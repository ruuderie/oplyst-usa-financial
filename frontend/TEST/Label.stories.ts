import type { Meta, StoryObj } from '@storybook/vue3';

import Label from '../components/ui/label/Label.vue';

const meta = {
  title: 'Label',
  component: Label,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof Label>;

export default meta;
type Story = StoryObj<typeof Label>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};