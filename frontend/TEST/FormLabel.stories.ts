import type { Meta, StoryObj } from '@storybook/vue3';

import FormLabel from '../components/ui/form/FormLabel.vue';

const meta = {
  title: 'FormLabel',
  component: FormLabel,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof FormLabel>;

export default meta;
type Story = StoryObj<typeof FormLabel>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};