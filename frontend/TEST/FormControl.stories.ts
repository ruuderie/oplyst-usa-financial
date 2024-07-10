import type { Meta, StoryObj } from '@storybook/vue3';

import FormControl from '../components/ui/form/FormControl.vue';

const meta = {
  title: 'FormControl',
  component: FormControl,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof FormControl>;

export default meta;
type Story = StoryObj<typeof FormControl>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};