import type { Meta, StoryObj } from '@storybook/vue3';

import FormItem from '../components/ui/form/FormItem.vue';

const meta = {
  title: 'FormItem',
  component: FormItem,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof FormItem>;

export default meta;
type Story = StoryObj<typeof FormItem>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};